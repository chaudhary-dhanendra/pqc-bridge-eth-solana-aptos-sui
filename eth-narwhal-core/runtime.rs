// src/runtime.rs
use anyhow::Result;
use std::sync::Arc;
use std::thread;
use tokio::runtime::{Builder, Runtime};
use tokio::runtime::Handle;

use crate::runtime_metrics::spawn_runtime_metrics_task;

pub struct TaskPools {
    pub core_handle: Handle,
    pub consensus_handle: Handle,
    pub networking_handle: Handle,
}

impl TaskPools {
    pub fn new() -> Result<(Self, Vec<thread::JoinHandle<()>>)> {
        // CORE runtime (RPC, admin, misc)
        let core_rt = Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .thread_name("core-worker")
            .build()?;

        // CONSENSUS runtime (DAG, Bullshark)
        let consensus_rt = Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .thread_name("consensus-worker")
            .build()?;

        // NETWORKING runtime (libp2p, sync)
        let networking_rt = Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .thread_name("net-worker")
            .build()?;

        let core_handle = core_rt.handle().clone();
        let consensus_handle = consensus_rt.handle().clone();
        let networking_handle = networking_rt.handle().clone();

        // Spawn a thread that owns the core runtime
        let core_join = thread::Builder::new()
            .name("core-rt".into())
            .spawn(move || {
                spawn_runtime_metrics_task(core_handle.clone());
                core_rt.block_on(async {
                    // Keep core runtime alive; actual tasks are spawned onto it from elsewhere.
                    futures::future::pending::<()>().await;
                });
            })?;

        // Spawn a thread that owns the consensus runtime
        let consensus_join = thread::Builder::new()
            .name("consensus-rt".into())
            .spawn(move || {
                spawn_runtime_metrics_task(consensus_handle.clone());
                consensus_rt.block_on(async {
                    futures::future::pending::<()>().await;
                });
            })?;

        // Spawn a thread that owns the networking runtime
        let networking_join = thread::Builder::new()
            .name("networking-rt".into())
            .spawn(move || {
                spawn_runtime_metrics_task(networking_handle.clone());
                networking_rt.block_on(async {
                    futures::future::pending::<()>().await;
                });
            })?;

        let pools = TaskPools {
            core_handle,
            consensus_handle,
            networking_handle,
        };

        Ok((pools, vec![core_join, consensus_join, networking_join]))
    }

    // Convenience helpers
    pub fn spawn_core<F>(&self, fut: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        self.core_handle.spawn(fut);
    }

    pub fn spawn_consensus<F>(&self, fut: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        self.consensus_handle.spawn(fut);
    }

    pub fn spawn_networking<F>(&self, fut: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        self.networking_handle.spawn(fut);
    }
}
