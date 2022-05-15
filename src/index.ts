import {createServer} from 'http';

const server = createServer((req, res) => {
  res.writeHead(200, {
    'Content-Type': 'text/plain',
  });
  res.write('no');
  res.end();
});

server.listen(8081);
