import React, {useState, useEffect} from 'react';
import './App.css';

function App() {
  const [search, setSearch] = useState("");
  const [photos, setPhotos] = useState(null);

  const onSearchChange = (e) => {
    let value = e.target.value.replace(/\s/g, '');
    setSearch(value);
  }

  const onSearchSubmit = (e) => {
    e.preventDefault();
    fetchSearchPhotos(search);
  }

  const fetchSearchPhotos = async (search) => {
    // setFetching(true);
    setPhotos(null);

    let res = await fetch(`/unsplash/search/photos?query=${search}`, {
      method: 'GET',
    });
    if (!res) return;

    let json = await res.json();
    if (!json) return;

    // setFetching(false);
    setPhotos(json);
  }

  return (
    <div className="app">
      <header><h1>Unsplash Client</h1></header>
      <form onSubmit={onSearchSubmit}>
        <input class="search" value={search} onChange={onSearchChange}></input>
      </form>
      {photos && photos.results.map((photo) => {
        return <img src={photo.urls.small} />;
      })}
    </div>
  );
}

export default App;
