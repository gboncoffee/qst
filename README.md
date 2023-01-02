# Quick Serve This

QST is a extremelly simple web server designed for serving static pages with an
instant startup.

**DISCLAIMER**: THIS IS NOT MEANT FOR PRODUCTION. THE EXPECTED USE CASE FOR THIS
SOFTWARE IS TO HELP THE DEVELOPMENT OF STATIC WEB PAGES, NOT FOR PRODUCTION.

## Installing

Install from [crates.io](https://crates.io/crates/qst):

`cargo install qst`

## Usage

`qst` have this cli args:

- `--port -p`: Choose a port to use. Defaults to `6969`.  
- `--addr -a`: Choose a IP address to bind on. Defaults to `127.0.0.1`.  
- `--default-file -f`: Choose a default file to send when fetching `/`. Defaults to
  `index.html`  
- `--err404-file -e`: Choose a file to send when returning a 404. Defaults to no
  file.  
- `--max-threads -t`: Limit the number of threads the server can spawn at the same
  time. Defaults to no limit.  
- `--limit-requests -l`: Limit the number of requests to respond. The server will
  exit when it reachs this number. Defaults to no limit.  

Examples calling with all args:

`qst --port 4200 --addr 192.168.0.1 --default-file home.html --err404-file err.html --max-threads 5 --limit-requests 10`  
`qst -p 4200 -a 192.168.0.1 -f home.html -e err.html -t 5 -l 10`  
