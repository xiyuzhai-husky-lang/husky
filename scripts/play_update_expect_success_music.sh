MONITOR=HDMI-2
DURATION=0.3
scripts/adjust_screen_brightness.sh 1.0 $MONITOR $DURATION
mpg321 assets/music/elden-ring-melina-voice-we-re-almost-there.mp3 >/dev/null 2>&1
