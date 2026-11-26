#!/usr/bin/env bash
# Run under xvfb-run in CI. Drive the shipped executable through normal input.
set -euo pipefail
out="${1:-dist/GameHub-demo.mp4}"
mkdir -p "$(dirname "$out")"
work=$(mktemp -d)
game_pid=''; capture_pid=''
cleanup() {
  if [[ -n "$capture_pid" ]]; then kill -INT "$capture_pid" 2>/dev/null || true; fi
  if [[ -n "$game_pid" ]]; then kill "$game_pid" 2>/dev/null || true; fi
  if [[ -n "${sink:-}" ]]; then pactl unload-module "$sink" 2>/dev/null || true; fi
  rm -rf "$work"
}
trap cleanup EXIT
pulseaudio --start --exit-idle-time=-1
sink=$(pactl load-module module-null-sink sink_name=gamehub_demo)
pactl set-default-sink gamehub_demo
cat > "$work/asound.conf" <<'ALSA'
pcm.!default { type pulse }
ctl.!default { type pulse }
ALSA
export PULSE_SINK=gamehub_demo ALSA_CONFIG_PATH="$work/asound.conf" XDG_DATA_HOME="$work/data" LIBGL_ALWAYS_SOFTWARE=1
./target/release/game-hub >"$work/game.log" 2>&1 & game_pid=$!
if ! wid=$(timeout 22 xdotool search --sync --onlyvisible --name '^GAME HUB$' | head -1); then
  cat "$work/game.log"; exit 1
fi
xdotool windowsize "$wid" 1120 720 windowmove "$wid" 0 0 windowfocus "$wid"
sleep 1
ffmpeg -nostdin -loglevel error -f x11grab -draw_mouse 0 -video_size 1120x720 -framerate 24 -i "$DISPLAY" -thread_queue_size 512 -f pulse -i gamehub_demo.monitor -c:a aac -b:a 96k -vf scale=896:576 -c:v libx264 -preset veryfast -pix_fmt yuv420p -movflags +faststart -y "$out" & capture_pid=$!
click() { xdotool mousemove --window "$wid" "$1" "$2" click 1; sleep 0.32; }
sleep 1
for card in 200 565 900; do
  click "$card" 360
  sleep 0.6
  click 155 621
  case "$card" in
    200) sleep 0.8; xdotool keydown w; sleep 0.3; xdotool keyup w keydown Down; sleep 0.28; xdotool keyup Down; sleep 1.4 ;;
    565) sleep 0.5; xdotool key s; sleep 0.45; xdotool key Right; sleep 0.55; xdotool key a; sleep 0.7 ;;
    900) xdotool key f Return u KP_Home; xdotool keydown d keydown Left; sleep 0.32; xdotool keyup d keyup Left; xdotool keydown s; sleep 0.32; xdotool keyup s; sleep 1.7 ;;
  esac
  xdotool key Escape; sleep 0.42; xdotool key BackSpace; sleep 0.45
done
# All -> page four, second card. Record the actual two-player rink through the launcher.
for _ in 1 2 3; do click 988 642; done
click 553 355
click 155 621
xdotool keydown d keydown Left; sleep 0.55; xdotool keyup d keyup Left
xdotool keydown s keydown Up; sleep 0.32; xdotool keyup s keyup Up
sleep 1.5
kill -0 "$game_pid" || { cat "$work/game.log"; exit 1; }
kill -INT "$capture_pid"; wait "$capture_pid" || [[ $? == 255 ]]; capture_pid=''
test -s "$out"
