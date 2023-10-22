cd ~
mkdir ./xhdf
cd ./xhdf
curl https://github.com/ryzmae/xhdf/releases/download/1.0.0/xhdf
chmod +x ./xhdf
echo "Please add the following line to your .bashrc or .zshrc or .bashrc file:"
echo "`export PATH="$HOME/.xhdf:$PATH"`"

# Path: install.sh