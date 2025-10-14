# Use the official Postgres image as base
FROM postgres:17

# Optional: set environment variables (you can override these at runtime)
ENV POSTGRES_USER=postgres
ENV POSTGRES_PASSWORD=password
ENV POSTGRES_DB=yayayum

# Optional: copy initialization SQL scripts into the image
# Any .sql or .sh file in this folder will run automatically on first startup
# Uncomment if you want to include initial data or schema
# COPY init-db/ /docker-entrypoint-initdb.d/

# Expose the default Postgres port
EXPOSE 5432
