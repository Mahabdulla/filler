FROM rust:1.78-buster

COPY ./maps			        /filler/maps
COPY ./linux_robots		    /filler/linux_robots
COPY ./linux_game_engine	/filler/linux_game_engine
COPY ./game			    /filler/game

RUN chmod +x /filler/linux_game_engine
RUN chmod +x /filler/linux_robots/*
RUN ln -s /filler/linux_game_engine /filler/game_engine
RUN ln -s /filler/linux_robots /filler/robots

# Fix Windows line endings in map files
RUN for map in /filler/maps/map*; do tr -d '\r' < "$map" > "${map}.tmp" && mv "${map}.tmp" "$map"; done

# Compile the student solution
WORKDIR /filler/game
RUN rm -f Cargo.lock
RUN cargo build --release

# Create symlink to the compiled binary
RUN ln -s /filler/game/target/release/filler /filler/student_player

WORKDIR /filler/

ENTRYPOINT ["/bin/bash"]