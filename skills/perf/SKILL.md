---
name: perf
description: Performance audit, latency benchmarking, and resource bottleneck identification for the workspace.
---

# Supercharger Performance Audit

Analyze critical execution paths, identify computational and I/O bottlenecks, and recommend optimization strategies.

## Analysis Checklist

1. **Hotspot Profiling**:
   - Trace hot loops and repeated calculations.
   - Detect redundant disk I/O, file reads, or synchronous network roundtrips.

2. **Memory & Allocation**:
   - Check for unnecessary heap allocations, unbounded caching, or object cloning.
   - Verify stream processing on large files instead of loading entire contents into memory.

3. **Concurrency & Async**:
   - Identify blocking calls inside async loops.
   - Verify appropriate thread pool utilization or batching strategies.

4. **Deliverable**:
   - Concrete latency/throughput measurements.
   - Ranked list of high-ROI optimizations with before/after diffs.
