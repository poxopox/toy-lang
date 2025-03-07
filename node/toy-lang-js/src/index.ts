import express from 'express';
import { createServer } from 'http';
import { Server } from 'socket.io';
import { Tokenizer } from 'toy-lang';
import path from 'path';

// Set up Express
const app = express();
const server = createServer(app);
const io = new Server(server);

// Serve static files
app.use(express.static(path.join(__dirname, '../public')));

// Socket.io connection handling
io.on('connection', (socket) => {
  console.log('Client connected');

  // Handle tokenization requests
  socket.on('tokenize', (code: string) => {
    try {
      const tokenizer = new Tokenizer(code);
      const tokens = tokenizer.tokenize();
      socket.emit('tokens', tokens);
      tokenizer.free();
    } catch (error) {
      console.error('Tokenization error:', error);
      socket.emit('error', { message: 'Failed to tokenize code' });
    }
  });

  socket.on('disconnect', () => {
    console.log('Client disconnected');
  });
});

// Start the server
const PORT = process.env.PORT || 3000;
server.listen(PORT, () => {
  console.log(`Server running on port ${PORT}`);
});
