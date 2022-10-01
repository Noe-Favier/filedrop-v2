# tls support : 

go in the folder `ssl` and then run : 

    openssl req -new -newkey rsa:4096 -x509 -sha256 -days 365 -nodes -out cert.crt -keyout key.key