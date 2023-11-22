@Library('jenkins-library@feature/DOPS-2746/rust_sonar_dojo_slither') _

def pipeline = new org.rust.AppPipeline(steps: this,
      envImageName: 'docker.soramitsu.co.jp/sora2/parachain-env:latest',
      appImageName: 'docker.soramitsu.co.jp/sora2/parachain',
      buildTestCmds: ['housekeeping/tests.sh'],
      cargoClippyTag: ':latest',
      cargoClippyCmds: ['housekeeping/clippy.sh'],
      codeCoverage: false,
      sonarProjectKey: 'sora:sora2-common',
      sonarProjectName: 'sora2-common',
      dojoProductType: 'sora'
)
pipeline.runPipeline()