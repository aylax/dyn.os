# lynn
music player

```sh
# about project
cargo run --example main # for examples/main.rs
cargo test # for test
```


```sh
# test upd 
cargo run lynn # udp socket listen on port 6000
# echo "send sth..." > /dev/udp/127.0.0.1/6000
exec 8<>/dev/udp/127.0.0.1/6000 # link dev/udp to file descriptor :8
echo "send sth...">&8 # write to fd :8
cat <&8 # out :8 content
```

```sh
# tree proj dir
tree -aF -I 'target|.git' > .proj
```
