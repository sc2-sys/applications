<div align="center">
  <h1><code>sc2-apps</code></h1>

  <p>
    <strong>Applications for
    <a href="https://github.com/sc2-sys/">Serverless Confidential Containers (SC2)</a></strong>
  </p>
  <hr>
</div>

This repository cotnains the workload applications that we use to evaluate SC2 as part of our
[experiments](https://github.com/sc2-sys/experiments).

All applications in SC2 are Knative applications, and we differenttiate between:
- [`./functions`](./functions) - individual Knative services.
- [`./workflows`](./workflows) - multi-function serverless workflows with Knative Eventing.

To upload all function and workflow images necessary for the experiments, just
run:

```bash
cargo run --release
```
