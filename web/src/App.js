import React, { useState, useEffect } from 'react';

function App() {
  const [fetchedString, setFetchedString] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(null);

  useEffect(() => {
    const fetchData = async () => {
      setIsLoading(true);
      try {
        const response = await fetch('http://localhost:8080/health');
        if (!response.ok) {
          throw new Error('Network response was not ok');
        }
        const text = await response.text();
        setFetchedString(text);
      } catch (error) {
        setError(error);
      } finally {
        setIsLoading(false);
      }
    };

    fetchData();
  }, []);

  return (
    <div>
      {isLoading && <p>Loading...</p>}
      {error && <p>Error: {error.message}</p>}
      {fetchedString && <p>{fetchedString}</p>}
    </div>
  );
}

export default App;
