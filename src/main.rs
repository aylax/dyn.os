use bevy::prelude::*;

struct Person;

struct Name(String);

fn hello_world() {
    println!("hello world!");
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, _app: &mut AppBuilder) {
        // add things to your app here
        println!("hello plugin");
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Elaina Proctor".to_string()));
    commands.spawn().insert(Person).insert(Name("Renzo Hume".to_string()));
    commands.spawn().insert(Person).insert(Name("Zayna Nieves".to_string()));
}

struct GreetTimer(Timer);

fn greet_people(
    time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}


fn main() {
    App::build()
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .add_startup_system(add_people.system())
        .add_system(hello_world.system())
        .add_system(greet_people.system())
        .run();
}

