import os
import subprocess
import platform
import sys

print("Starting development environment...")
print("Press CTRL+C twice to exit")

os.environ['ENVIRONMENT'] = 'DEV'

if platform.system() == 'Windows':
    subprocess.run(['cmd.exe', '/c', 'title', 'Tailwind'])
else:
    print('\033]0;Tailwind\007', end='', flush=True)

tailwind_process = subprocess.Popen([
    'npx', 'tailwindcss', 
    '-i', './static/input.css', 
    '-o', './static/style.css', 
    '--minify', 
    '-w'
], shell=True)

subprocess.run([
    'cargo', 'watch', 
    '-w', 'src', 
    '-w', 'Cargo.toml', 
    '-w', 'templates', 
    '-w', 'content', 
    '-w', 'static', 
    '-x', 'run'
])

# Cleanup after cargo watch exits
print("Shutting down development environment...")
tailwind_process.terminate()

print("Resetting console...")
if platform.system() == 'Windows':
    subprocess.run(['cmd.exe', '/c', 'mode', 'con:'])
else:
    subprocess.run(['reset'])

print("Development environment stopped.")