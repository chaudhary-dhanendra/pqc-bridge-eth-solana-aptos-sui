use std::future::Future;
use std::sync::Arc;

use tokio::runtime::Handle;
use tokio::task::JoinHandle;
use tokio::sync::Semaphore;

/// Logical “pools” on top of a single Tokio runtime.
/// Each pool is just a semaphore-bounded spawn helper.
///
/// - `spawn_network`   → for libp2p / networking / I/O heavy tasks
/// - `spawn_consensus` → for Narwhal / Bullshark / ordering tasks
#[derive(Clone)]
pub struct TaskPools {
    handle: Handle,
    network_limit: Arc<Semaphore>,
    consensus_limit: Arc<Semaphore>,
}

impl TaskPools {
    /// Create new logical task pools.
    ///
    /// `network_limit` and `consensus_limit` are the maximum number of
    /// concurrent tasks allowed in each pool.
    pub fn new(network_limit: usize, consensus_limit: usize) -> Self {
        Self {
            handle: Handle::current(),
            network_limit: Arc::new(Semaphore::new(network_limit)),
            consensus_limit: Arc::new(Semaphore::new(consensus_limit)),
        }
    }

    /// Spawn a “network” task (e.g. P2P, RPC client, gossip, I/O loops).
    pub fn spawn_network<F>(&self, fut: F) -> JoinHandle<()>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let handle = self.handle.clone();
        let semaphore = self.network_limit.clone();

        handle.spawn(async move {
            let _permit = semaphore
                .acquire_owned()
                .await
                .expect("network semaphore closed");
            fut.await;
        })
    }

    /// Spawn a “consensus” task (e.g. DAG building, Bullshark, ordering).
    pub fn spawn_consensus<F>(&self, fut: F) -> JoinHandle<()>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let handle = self.handle.clone();
        let semaphore = self.consensus_limit.clone();

        handle.spawn(async move {
            let _permit = semaphore
                .acquire_owned()
                .await
                .expect("consensus semaphore closed");
            fut.await;
        })
    }
}
