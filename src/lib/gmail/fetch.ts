 const token = window.localStorage.getItem('oauth_provider_token');

const messages = fetch('https://gmail.googleapis.com/gmail/v1/users/me/messages/1913ed2be6640574', {
    headers: {
        Authorization: `Bearer ${token}`,
    },
})
    .then(response => response.json())
    .then(async data =>  {
        // Handle the response data here
        console.log(data);
    })
    .catch(error => {
        // Handle any errors here
        console.error(error);
    });

export default messages