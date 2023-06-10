#!/bin/bash

#enable i2c, raspi-config should be preinstalled on all popular OS, if you use other way to enable i2c, just comment out this
sudo raspi-config nonint do_i2c 0

#copy configuration file if exist
if test -f "argon_fan_controller_cfg.yml"; then
    sudo cp ./argon_fan_controller_cfg.yml /etc/
fi

#copy main executable
sudo chmod 755 ./argon_fan_controller
sudo cp ./argon_fan_controller /usr/bin/

#copy service file
sudo chmod 644 ./argon_fan_controller.service
sudo cp ./argon_fan_controller.service /lib/systemd/system/

#run service
sudo systemctl enable /lib/systemd/system/argon_fan_controller.service
sudo systemctl start argon_fan_controller