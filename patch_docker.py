with open('/Users/rasol/DevsTools/codes/flutter/taqati/rust/taqati_analytics/Dockerfile', 'r') as f:
    content = f.read()

content = content.replace('FROM rust:1.75 AS builder', 'FROM rust:latest AS builder')

with open('/Users/rasol/DevsTools/codes/flutter/taqati/rust/taqati_analytics/Dockerfile', 'w') as f:
    f.write(content)
