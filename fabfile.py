from fabric.api import cd, env, local, sudo, task, run

env.hosts = ["lidar.io"]
env.use_ssh_config = True
REMOTE_DIRECTORY = "/var/www/atlas.lidar.io"

@task
def deploy():
    local("cargo build")
    local("cargo test")
    with cd(REMOTE_DIRECTORY):
        run("git pull")
        run("cargo build --release")
        sudo("supervisorctl restart atlas.lidar.io")
