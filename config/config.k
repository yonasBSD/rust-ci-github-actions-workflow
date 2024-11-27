schema Nginx:
    """Schema for Nginx configuration files"""
    http: Http

schema Http:
    server: Server

schema Server:
    listen: int | str    # The attribute `listen` can be int type or a string type.
    location?: Location  # Optional, but must be non-empty when specified

    check:
        listen >= 80, "port must be >= 80"

schema Location:
    root: str
    index: str

nginx = Nginx {
    http.server = {
        listen = 80
        location = {
            root = "{{ root }}"
            index = "index.html"
        }
    }
}

env: str = option("env") or "dev"  # The attribute `env` has a default value "den"
database: str = option("database")
hosts = {
    dev = "postgres.dev"
    stage = "postgres.stage"
    prod = "postgres.prod"
}
dbConfig = {
    host = hosts[env]
    database = database
    port = "2023"
    conn = "postgres://${host}:${port}/${database}"
}
