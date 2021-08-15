# Arch Linux Desktop Setup Scripts and Configs
This repo contains all of my personal Arch Linux configs for i3, polybar, etc. Running the install script in the root of the repo should install all components (requires user input during installation of various components).
## General Arch Installation Guide
This section contains steps to install Arch in general (prior to running these install scripts). These steps were generally created from https://wiki.archlinux.org/title/installation_guide with some changes based on personal experience.

- Ensure network connection works by pinging some ip/domain.
- Enable system clock synchronization with `timedatectl set-ntp true`.
- Run `lsblk` or `fdisk -l` to see available disks, then run `parted` to start partitioning. You can use the `select` parted command to select a disk.
    - Create a partition table with `mklabel gpt`.
    - If using legacy BIOS, create a BIOS partition like so: `mkpart bios 0% 4MB` and `set 1 bios_grub on`
    - If using UEFI, create a EFI partition like so: `mkpart efi 0% 512MB` (you can go smaller if needed) and `set 1 boot on` and `set 1 esp on`.
    - Create swap partition (optional but recommended) with `mkpart swap 4MB 4MB+SIZE` on BIOS and `mkpart swap 512MB 512MB+SIZE` on UEFI. Mark as swap: `set 2 swap on`. Replace size with the desired size of the swap partition (see: https://itsfoss.com/swap-size/) and do the actual addition calculation by hand (don't actually type `+`).
    - Create the main partition with `mkpart primary 4MB+SIZE 100%` or `mkpart primary 512MB+SIZE 100%`.
    - You're done with parted, go ahead and `quit`.
    - Go ahead and run `lsblk` to see partitions again for the next steps.
    - Format the EFI parition with `mkfs.fat -F 32 /dev/partition` (no need to format the BIOS partition).
    - Format the swap partition with `mkswap /dev/partition`.
    - Format the main partition with `mkfs.ext4 /dev/partition`.
    - Mount the main partition with `mount /dev/partition /mnt`.
    - Enable the swap partition with `swapon /dev/partition`.
    - Run pacstrap: `pacstrap /mnt base base-devel linux linux-firmware`.
    - Generate the fstab file: `genfstab -U /mnt >> /mnt/etc/fstab`. Should probably also check to make sure it generated correctly.
    - Time to chroot: `arch-chroot /mnt`.
    - Set time zone: `ln -sf /usr/share/zoneinfo/America/New_York /etc/localtime` (or whatever time zone you want).
    - Generate adjtime: `hwclock --systohc`.
    - Install nano with `pacman -S nano` and uncomment `en_US.UTF-8 UTF-8` from `/etc/locale.gen` (tip: use ctrl+w in nano to search for something). Then generate locale: `locale-gen`.
    - Create locale config: `echo 'LANG=en_US.UTF-8' > /etc/locale.conf`.
    - Create hostname: `echo 'myhostname' > /etc/hostname` (whatever hostname you want) and write
        ```
        127.0.0.1	localhost
        ::1		localhost
        127.0.1.1	myhostname.localdomain	myhostname
        ```
      into `/etc/hosts`.
    - Set the root password with `passwd`.
    - Install and enable dhcpcd so that wired network connections work (more may be needed for wifi, so just used wired for first install): `pacman -S dhcpcd` and `systemctl enable dhcpcd`.
    - Enable sudo user group: `EDITOR=nano visudo` and uncomment the line enabling `wheel` as a user group for sudo.
    - Create your user: `useradd -m user` and set its password: `passwd user`. Add it to the sudo group: `gpasswd -a user wheel`.
    - Install grub: `pacman -S grub`. Also install `efibootmgr` if using UEFI.
        - For BIOS: `grub-install --target=i386-pc /dev/drive` (the *drive*, not the *partition*)
        - For UEFI: First mount the EFI partition to `/efi`: `mkdir /efi` and `mount /dev/partition /efi`, then `grub-install --target=x86_64-efi --efi-directory=/efi --bootloader-id=GRUB`.
        - Generate grub config: `grub-mkconfig -o /boot/grub/grub.cfg`.
    - Now, `reboot`, log in with your user, ensure sudo and other components work, clone this repo, and run the install script.