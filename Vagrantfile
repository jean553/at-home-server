# -*- mode: ruby -*-
# vi: set ft=ruby ts=2 sw=2 expandtab :

PROJECT = "at-home-server"

DOCKER_ENV = {
  "HOST_USER_UID" => Process.euid,
  "APP_PATH" => "/vagrant/at-home-server",
  "REDIS_URL" => "redis://at-home-server_db/",

  "AWS_ACCESS_KEY_ID" => ENV["AWS_ACCESS_KEY_ID"],
  "AWS_SECRET_ACCESS_KEY" => ENV["AWS_SECRET_ACCESS_KEY"]
}

ENV['VAGRANT_NO_PARALLEL'] = 'yes'
ENV['VAGRANT_DEFAULT_PROVIDER'] = 'docker'
Vagrant.configure(2) do |config|

  config.vm.define "db" do |db|
    db.vm.provider "docker" do |d|
      d.image = "redis"
      d.name = "#{PROJECT}_db"
    end
  end

  config.ssh.insert_key = false
  config.vm.define "dev", primary: true do |app|
    app.vm.provider "docker" do |d|
      d.image = "jean553/rust-dev-docker"
      d.name = "#{PROJECT}_dev"
      d.link "#{PROJECT}_db:db"
      d.has_ssh = true
      d.env = DOCKER_ENV
    end
    app.vm.network "forwarded_port", guest: 8000, host: 8000
    app.ssh.username = "vagrant"
  end
end
