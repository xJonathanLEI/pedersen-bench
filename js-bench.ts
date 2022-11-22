import Benchmark from "benchmark";
import * as micro_starknet from "micro-starknet";

var suite = new Benchmark.Suite();

suite
  .add("paulmillr/micro-starknet@a1e7fa0", function () {
    micro_starknet.pedersen(
      "0x03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb",
      "0x0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a"
    );
  })
  .on("cycle", (event: any) => {
    console.log(`${event.target.name}: ${event.target.stats.mean}`);
  })
  .run({ async: true });
