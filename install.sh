#!/bin/sh
main=rssplugin

cp lib${main}.so /usr/lib/xfce4/panel-plugins/
cp ${main}.desktop /usr/share/xfce4/panel-plugins/
cp ${main}.svg /usr/share/icons/hicolor/scalable/apps/
update-icon-caches /usr/share/icons/hicolor/scalable/apps/${main}.svg
