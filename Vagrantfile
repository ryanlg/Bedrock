# -*- mode: ruby -*-
# vi: set ft=ruby :

# All Vagrant configuration is done below. The "2" in Vagrant.configure
# configures the configuration version (we support older styles for
# backwards compatibility). Please don't change it unless you know what
# you're doing.
Vagrant.configure("2") do |config|
  config.vm.box = "hashicorp/bionic64"

  config.vm.provider "virtualbox" do |v|
    v.memory = 2048
    v.cpus = 8
  end

  config.vm.network "forwarded_port", guest: 1234, host: 1234

  config.vm.provision :shell, path: "support/vagrant/privileged.sh", privileged: true
  config.vm.provision :shell, path: "support/vagrant/bootstrap.sh",  privileged: false
end
