const albumList = document.getElementById('album-list');
const albumName = document.getElementById('album-name');
const albumTracks = document.getElementById('album-tracks');
const albumArtists = document.getElementById('album-artists');
const albumGenres = document.getElementById('album-genres');
const albumYear = document.getElementById('album-year');
const albumRating = document.getElementById('album-rating');
const albumComment = document.getElementById('album-comment');
const commentList = document.getElementById('comment-list');

// Fetch albums and display them in the list
fetch('http://localhost:8080/albums')
  .then(response => response.json())
  .then(data => {
    data.forEach(album => {
      const li = document.createElement('li');
      li.textContent = album.name;
      li.addEventListener('click', () => displayAlbumDetails(album.album_id));
      albumList.appendChild(li);
    });
  })
  .catch(error => console.error('Error fetching albums:', error));

// Display album details
function displayAlbumDetails(albumId) {
  fetch(`http://localhost:8080/albums/${albumId}`)
    .then(response => response.json())
    .then(album => {
      albumName.textContent = album.name;
      albumTracks.textContent = album.tracks;
      albumArtists.textContent = album.artists;
      albumGenres.textContent = album.genres;
      albumYear.textContent = album.year;
      albumRating.textContent = album.rating;
      albumComment.textContent = album.comment;
      fetchComments(albumId);
    })
    .catch(error => console.error(`Error fetching album ${albumId}:`, error));
}

// Fetch comments for an album and display them in the list
function fetchComments(albumId) {
  fetch(`http://localhost:8080/albums/${albumId}/comments`)
    .then(response => response.json())
    .then(comments => {
      commentList.innerHTML = '';
      comments.forEach(comment => {
        const li = document.createElement('li');
        li.textContent = comment.text;
        commentList.appendChild(li);
      });
    })
    .catch(error => console.error(`Error fetching comments for album ${albumId}:`, error));
}