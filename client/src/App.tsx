import { For, createResource } from 'solid-js';
import { effect } from 'solid-js/web';
import './App.css';
import { Person } from './types';

const AVATAR_1 =
  "https://res.cloudinary.com/dqse2txyi/image/upload/v1666049372/axum_server/img_avatar_lf92vl.png";

const AVATAR_2 =
  "https://res.cloudinary.com/dqse2txyi/image/upload/v1666049372/axum_server/img_avatar2_erqray.png";

const fetchPeople = async () => (await fetch('http://localhost:3000/people')).json()

function App() {
  const [people] = createResource<Person[]>(fetchPeople);
  effect(() => console.log(people()));


  return (
    <div class='app'>
      <For each={people()}>{(person, index) =><div  class="card">
          <img src={index() % 2 == 0 ? AVATAR_1 : AVATAR_2} alt="Avatar" />
          <div class="container">
            <h4>
              <b>{person.name}</b>
            </h4>
            <p>Age: {person.age}</p>
            <p>Favourite Food: {person.favorite_food ?? "Unknown"}</p>
          </div>
        </div>}</For>
    </div>
  )
}

export default App
