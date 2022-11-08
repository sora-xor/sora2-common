@Library('jenkins-library' ) _

def pipeline = new org.js.LibPipeline( steps: this,
    packageManager: "pnpm",
    buildDockerImage: 'build-tools/node:16-rust-1.62',
    testCmds: ['cargo test --features runtime-benchmarks'],
    secretScannerExclusion: '.*Cargo.toml'
   )
pipeline.runPipeline()