import React, {useState} from 'react';
import './App.css';

function App() {
  const [search, setSearch] = useState("");
  const [photos, setPhotos] = useState(null);
  const [limitInfo, setLimitInfo] = useState(null);

  const onSearchChange = (e) => {
    setSearch(e.target.value);
  }

  const onSearchSubmit = (e) => {
    e.preventDefault();
    fetchSearchPhotos(search);
  }

  const validate_res = async (res) => {
    if (!res) return null;
    let json = await res.json();
    if (!json) return null;
    return json;
  }

  // useEffect(() => {
  //   async function fetchLimits() {
  //     let res = await fetch(`/unsplash/limit-info`, {
  //       method: 'GET',
  //     });
  //     let json = await validate_res(res);
  //     setLimitInfo(json);
  //   }
  //   fetchLimits();
  // }, [photos]);

  const fetchSearchPhotos = async () => {
    let res = await fetch(`/unsplash/search/photos?&query=${search}`, {
      method: 'GET',
    });
    let json = await validate_res(res);
    setPhotos(json.body);
    setLimitInfo(json.headers);
  }

  const fetchPhotosRandom = async () => {
    let url = "/unsplash/photos/random?count=1";
    url = search ? url + `&query=${search}` : url;

    let res = await fetch(url, {
      method: 'GET',
    });

    let json = await validate_res(res);
    setPhotos({results: json.body});
    setLimitInfo(json.headers);
  }

  return (
    <div className="app">
      <header><h1>Unsplash Client</h1></header>
      <form onSubmit={onSearchSubmit}>
        <input class="search" value={search} onChange={onSearchChange}></input>
      </form>
      <div id="buttons">
        <button onClick={fetchPhotosRandom}>Random</button>
        {limitInfo && <span>Limit: {limitInfo["X-Ratelimit-Limit"]}</span>}
        {limitInfo && <span>Remaining: {limitInfo["X-Ratelimit-Remaining"]}</span>}
      </div>
      <section id="photos">
        {photos && photos.results.map((photo) => {
          return <a href={photo.urls.regular} target="_blank" rel="noopener noreferrer">
            <div className="info">
              <p>{photo.description}</p>
              <p>{photo.user.name}</p>
            </div>
            <img alt="result" src={photo.urls.small} />
          </a>;
        })}
      </section>
    </div>
  );
}

export default App;
