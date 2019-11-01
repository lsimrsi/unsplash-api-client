import React, {useState, useEffect} from 'react';
import './App.css';

function App() {
  const [search, setSearch] = useState("");
  const [photos, setPhotos] = useState(null);
  const [limitInfo, setLimitInfo] = useState({
    limit: 0,
    remaining: 0,
  });

  const onSearchChange = (e) => {
    setSearch(e.target.value);
  }

  const onSearchSubmit = (e) => {
    e.preventDefault();
    fetchSearchPhotos(search);
  }

  useEffect(() => {
    async function fetchLimits() {
      let res = await fetch(`/unsplash/limit-info`, {
        method: 'GET',
      });
      if (!res) return;
  
      let json = await res.json();
      if (!json) return;
  
      setLimitInfo(json);
    }
    fetchLimits();
  }, [photos]);

  const fetchSearchPhotos = async () => {
    // setFetching(true);
    setPhotos(null);

    let res = await fetch(`/unsplash/search/photos?&query=${search}`, {
      method: 'GET',
    });
    if (!res) return;

    let json = await res.json();
    if (!json) return;

    // setFetching(false);
    setPhotos(json);
  }

  const fetchPhotosRandom = async () => {
    // setFetching(true);
    setPhotos(null);

    let url = "/unsplash/photos/random?count=1";
    url = search ? url + `&query=${search}` : url;
  
    let res = await fetch(url, {
      method: 'GET',
    });
    if (!res) return;

    let json = await res.json();
    if (!json) return;

    // setFetching(false);
    setPhotos({results: json});
  }

  return (
    <div className="app">
      <header><h1>Unsplash Client</h1></header>
      <form onSubmit={onSearchSubmit}>
        <input class="search" value={search} onChange={onSearchChange}></input>
      </form>
      <div id="buttons">
        <button onClick={fetchPhotosRandom}>Random</button>
      </div>
      <span>Limit: {limitInfo.limit}</span>
      <span>Remaining: {limitInfo.remaining}</span>
      <section id="photos">
        {photos && photos.results.map((photo) => {
          return <img alt="result" src={photo.urls.small} />;
        })}
      </section>
    </div>
  );
}

export default App;
