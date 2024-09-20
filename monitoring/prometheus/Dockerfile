# Copy Prometheus configuration
# COPY prometheus.yml /prometheus/prometheus.yml 
# Better keep this outside the volume

# Use the official Prometheus base image
FROM prom/prometheus:latest

# Set the working directory
WORKDIR /prometheus

# Expose the Prometheus port
EXPOSE 9090

# Specify the config file location (optional, if not using volume mounting)
CMD ["--config.file=/etc/prometheus/prometheus.yml"]