#!/bin/bash

# Entfernen der ausführbaren Datei aus /usr/local/bin
sudo rm /usr/local/bin/gm

# Entfernen der ausführbaren Datei aus ~/bin
rm ~/bin/gm

# Entfernen des Pfads aus ~/.bashrc, falls er existiert
sed -i.bak '/export PATH="\$HOME\/bin:\$PATH"/d' ~/.bashrc

# Sicherstellen, dass die Änderungen geladen werden
source ~/.bashrc
