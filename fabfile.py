from fabric.api import cd, env, local, sudo, task, run

env.hosts = ["lidar.io"]
env.use_ssh_config = True

@task
def deploy():
    local("cargo build")
    local("cargo test")
    with cd("/var/www/atlas.lidar.io"):
        run("git pull")
        run("cargo build --release")
        sudo("supervisorctl restart atlas.lidar.io")

@task
def reload():
    sudo("supervisorctl reload")
