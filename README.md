# wse
Simple web-server. Exec wse in directory and all files are serve on localhost

## Install
Only MacOS
```
brew tap ErmolaevID/wse
brew install wse
```

## Usage
```
wse 
```
start serve your current directory

```
wse ./hello
```
start serve along the way ./hello relative to your current directory

```
wse -p 4040
```
start serve your current directory with 4040 port. Default port is 7878

## How works
You're in a directory with the following structure
```
├── src
│   ├── index.js
├── public
│   ├── index.html
└── text.txt
```
You can exec `wse` in this directory. Then starts web-server that serve all files. 
```
curl localhost:7878/public/index.html
curl localhost:7878/src/index.js
curl localhost:7878/text.txt
```
