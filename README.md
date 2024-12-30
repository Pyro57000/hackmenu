# Hackmenu
A quick little tool to manage which projects you're on and manage distrobox containers for each environment.

In order to use this tool you'll want to have distrobox set up and have a "template" box you've created that has all yoru tools installed and what not, you'll likely also want to have a folder full of your other custom tools that you get from git hub, this makes it easier.

The distroboxes it sets up will have the prjoect files folder for the project mounted at /pentest and folder you use for custom tools (like the ones you clone from github) at /tools so getting to your files for the project is as easy as `cd /pentest`!

# General Use case and flow
I'm not very good at organization. In order to keep track of all the things needed for hacking projects and keep attack data separated from other attack data I worte this tool to do it for me. Basically I have a distrobox for each project, a folder to keep files related to the project, and a separate folder full of my markdown notes for the project. An example is below

Current engagements: client1 internal pentest, client 2 internal pentest

/home/pyro/
- CTFs
    - HTB
      - ctf1
      - ctf2
    - tryhackme
      - ctf1
      - ctf2
    - defcon
      - ctf1
      - ctf2
    - homelab
      - ad practice
      - linux privesc lab   

- notes
  - ctfs
    - htb
      - ctf1
      - ctf2
    - tryhackme
      - ctf1
      - ctf2
    - defcon
      - ctf1
      - ctf2
    - homelab
      - ad practice
      - linux privesclab 
- tools
  - bloodhound-linux-x86_64_4.3.1
  - bofhound
  - burp_extensions
  - ek45
  - etc

This tool automatically creates the file structure.  

For example if I'm starting a new CTF project I'll use option 5 to import and setup a new distrobox.

It will prompt me for a category (real, ctfs, other) in this case I'll choose ctfs.

It will them prompt for an entitiy name, this is the entity that is providing the CTF, for exmple htb.

it will then prompt for the project name - I usually just keep it the same as the CTF name ex ctf1.

It will ask if you have note or files to copy over, generally I won't have these yet, so I just answer no.

It will then create the folder structure indicated above and duplicate the template distrobox into a dedicated distrobox for this CTF. After this completes the project is set up and ready to go!

# Installation
1. download the latest executable from the releases in github.
2. copy this executable to a folder on your $PATH such as /usr/bin.
3. ensure you have a folder to keep your project files in created.
4. ensure you have a foldder to keep your notes in created.
5. ensure you have a distrobox created with all the tools and configurations you want. 
6. run the tool and follow the on screen prompts.  

# Manual Building instructions
1. clone this repository `git clone https://github.com/Pyro57000/hackmenu.git`
2. cd into the nested "hackmenu" folder `cd hackmenu`
3. use cargo to build the release binary `cargo build --release`
4. follow the same installation instructions, skipping the step where you download the release binary.
