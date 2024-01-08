FROM rust:1.67

WORKDIR /home/archi/Documents/rust_learning
COPY . .

RUN cargo install --path .

CMD ["rust_learning"]
