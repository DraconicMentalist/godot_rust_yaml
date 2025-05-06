from sh import cargo
import os
import shutil


def make_binary_folder(path: str, folder: str):
    folder_path = f'{path}/addons/godot_rust_yaml/{folder}'
    os.makedirs(folder_path, exist_ok=True)

## TODO: Mordred: I may want to come back and clean up this code later! It seems a bit messy.
def main():
    working_dir = os.path.dirname(os.path.abspath(__file__)) 
    os.chdir(f"{working_dir}/rust")
    linux_toolchain = 'x86_64-unknown-linux-gnu'
    windows_toolchain = 'x86_64-pc-windows-gnu'
    cargo.build('--release', '--target', linux_toolchain, _fg=True)
    cargo.build('--release', '--target', windows_toolchain, _fg=True)
    windows_dir = 'windows.x86_64'
    linux_dir = 'linux.x86_64'
    print("------ Build done! Now moving generated files to the correct addon directory!")
    os.chdir(working_dir)
    make_binary_folder(working_dir, linux_dir)
    make_binary_folder(working_dir, windows_dir)
    shutil.move(f'{working_dir}/rust/target/{windows_toolchain}/release/godot_rust_yaml.dll', f'{working_dir}/addons/godot_rust_yaml/{windows_dir}/')
    shutil.move(f'{working_dir}/rust/target/{linux_toolchain}/release/libgodot_rust_yaml.so', f'{working_dir}/addons/godot_rust_yaml/{linux_dir}/')
    print('Build Complete!')

if __name__ == "__main__":
    main()
