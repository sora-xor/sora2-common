@Library('jenkins-library@feature/dops-2942-update_rust_lib') _

def pipeline = new org.rust.AppPipeline(steps: this,
      envImageName: 'docker.soramitsu.co.jp/sora2/env:env',
      appImageName: 'docker.soramitsu.co.jp/sora2/parachain',
      buildTestCmds: 'housekeeping/tests.sh',
      disableCodeCoverage: true,
      clippyLinter: false
)
pipeline.runPipeline()