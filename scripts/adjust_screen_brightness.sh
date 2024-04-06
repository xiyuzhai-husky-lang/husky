#!/bin/bash

# Target brightness level (0.0 to 1.0)
TARGET_BRIGHTNESS=$1
# Output device, e.g., HDMI-1-0, eDP-1, etc.
OUTPUT_DEVICE=$2
# Duration over which to change the brightness, in seconds
DURATION=$3

# Get current brightness
CURRENT_BRIGHTNESS=$(xrandr --verbose | grep -m 1 -i brightness | cut -f2 -d ' ')

# Calculate steps for smooth transition
STEPS=100
SLEEP_DURATION=$(echo "$DURATION / $STEPS" | bc -l)

# Calculate brightness difference
BRIGHTNESS_DIFF=$(echo "$TARGET_BRIGHTNESS - $CURRENT_BRIGHTNESS" | bc -l)
STEP_BRIGHTNESS=$(echo "$BRIGHTNESS_DIFF / $STEPS" | bc -l)

for (( i=0; i<$STEPS; i++ )); do
  # Calculate new brightness
  NEW_BRIGHTNESS=$(echo "$CURRENT_BRIGHTNESS + $STEP_BRIGHTNESS * $i" | bc -l)
  # Apply new brightness
  xrandr --output $OUTPUT_DEVICE --brightness $NEW_BRIGHTNESS
  # Sleep for a fraction of the total duration
  sleep $SLEEP_DURATION
done

# Ensure target brightness is set at the end (to correct for any rounding errors)
xrandr --output $OUTPUT_DEVICE --brightness $TARGET_BRIGHTNESS
