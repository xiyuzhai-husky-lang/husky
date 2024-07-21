MONITOR=HDMI-2
DURATION=0.3
scripts/adjust_screen_brightness.sh 1.0 $MONITOR $DURATION
mpg321 assets/music/elden-ring-you-died-sound-effect.mp3 >/dev/null 2>&1
exit 1
