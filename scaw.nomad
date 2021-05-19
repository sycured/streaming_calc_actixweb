job "scaw" {
  datacenters = ["dc1"]
  type = "service"

  group "scaw" {
    count = 1
    network {
      port "http" {}
    }

    service {
      name = "scaw"
      tags = ["rust", "compute", "rest"]
      port = "http"
      check {
        name     = "alive"
        type     = "http"
        path     = "/"
        interval = "5s"
        timeout  = "2s"
      }
    }

    task "scaw" {
      driver = "docker"
      config {
        image = "ghcr.io/sycured/scaw:latest"
        ports = ["http"]
      }
      resources {
        cpu    = 100
        memory = 64
      }
      template {
        data        = <<EOH
        APP_IP="0.0.0.0"
        APP_PORT={{ env "NOMAD_PORT_http" }}
        EOH
        destination = "local/scaw.env"
        env         = true
      }
    }
  }
}
