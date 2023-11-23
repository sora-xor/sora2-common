@Library('jenkins-library') _

def pipeline = new org.rust.AppPipeline(steps: this,
      envImageName: 'docker.soramitsu.co.jp/sora2/parachain-env:latest',
      appImageName: 'docker.soramitsu.co.jp/sora2/parachain',
      buildTestCmds: ['housekeeping/tests.sh'],
      cargoClippyTag: ':latest',
      cargoClippyCmds: ['housekeeping/clippy.sh'],
      codeCoverage: true,
      codeCoverageCommand: './housekeeping/coverage.sh',
      sonarProjectKey: 'sora:sora2-common',
      sonarProjectName: 'sora2-common',
      dojoProductType: 'sora'
)
pipeline.runPipeline()