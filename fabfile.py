from fabric.api import local, put, env, task

REMOTE_DIRECTORY = "/var/www/atlas.lidar.io"

env.hosts = ["lidar.io"]
env.use_ssh_config = True


@task
def deploy():
    build()
    upload()


@task
def build():
    local("ng build --prod")


@task
def upload():
    put("dist/*", REMOTE_DIRECTORY)
