use sc_executor::native_executor_instance;
pub use sc_executor::NativeExecutor;

// Declare an instance of the native executor named `Executor`. Include the wasm binary as the
// equivalent wasm code.
native_executor_instance!(
    pub Executor,
    substrate_node_runtime::api::dispatch,
    substrate_node_runtime::native_version,
    frame_benchmarking::benchmarking::HostFunctions,
);
