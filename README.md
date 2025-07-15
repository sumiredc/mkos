```sh
cd wasabi
```

```sh
# ビルド
cargo build --target x86_64-unknown-uefi

# 実行ファイルをコピー
cp target/x86_64-unknown-uefi/debug/wasabi.efi mnt/EFI/BOOT/BOOTX64.EFI

# UEFI を起動
qemu-system-x86_64 -bios third_party/ovmf/RELEASEX64_OVMF.fd -drive format=raw,file=fat:rw:mnt  
```

