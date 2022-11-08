@Library('jenkins-library')

String agentLabel             = 'docker-build-agent'
String registry               = 'docker.soramitsu.co.jp'
String dockerBuildToolsUserId = 'bot-build-tools-ro'
String cargoImage        = registry + '/build-tools/cargo_audit'
String secretScannerExclusion = '.*Cargo.toml'
Boolean disableSecretScanner  = false

pipeline {
    options {
        buildDiscarder(logRotator(numToKeepStr: '20'))
        timestamps()
        disableConcurrentBuilds()
    }
    agent {
        label agentLabel
    }
    stages {
        stage('Secret scanner') {
            steps {
                script {
                    gitNotify('main-CI', 'PENDING', 'This commit is being built')
                    docker.withRegistry('https://' + registry, dockerBuildToolsUserId) {
                        secretScanner(disableSecretScanner, secretScannerExclusion)
                    }
                }
            }
        }
        stage('Init submodule') {
            environment {
                GIT_SSH_COMMAND = "ssh -o UserKnownHostsFile=/dev/null StrictHostKeyChecking=no"
            }
            steps {
                script {
                    sshagent(['soramitsu-bot-ssh']) {
                        sh """
                            git submodule update --init --recursive
                        """
                    }
                }
            }
        }
        stage('Tests') {
            steps {
                script {
                    docker.withRegistry( 'https://' + registry, dockerBuildToolsUserId) {
                        docker.image(cargoImage + ':latest').inside(){
                                sh """
                                    cargo test  --release --features runtime-benchmarks
                                """
                        }
                    }
                }
            }
        }
    }
    post {
        always {
            script{
                gitNotify('main-CI', currentBuild.result, currentBuild.result)
            }
        }
        cleanup { cleanWs() }
    }
}