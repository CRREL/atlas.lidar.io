from fabric.api import cd, env, local, sudo, task, run

env.hosts = ["lidar.io"]
env.use_ssh_config = True

@task
def deploy(branch="master"):
    local("git checkout {}".format(branch))
    local("cargo build")
    local("cargo test")
    with cd("/var/www/atlas.lidar.io"):
        run("git fetch origin")
        run("git checkout {}".format(branch))
        run("git pull")
        run("cargo build --release")
        sudo("supervisorctl restart atlas.lidar.io")

@task
def reload():
    sudo("supervisorctl reload")
