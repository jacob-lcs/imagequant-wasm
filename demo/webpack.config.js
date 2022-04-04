const HtmlWebpackPlugin = require('html-webpack-plugin')

module.exports = {
  entry: './index.tsx',
  plugins: [new HtmlWebpackPlugin()],
  mode: 'development',
  module: {
    rules: [
      {
        test: /\.(tsx|ts|jsx|js)?$/,
        use: ["babel-loader"]
      },
    ]
  },
  resolve: {
    extensions: [
      '.web.mjs',
      '.mjs',
      '.web.js',
      '.js',
      '.web.ts',
      '.ts',
      '.web.tsx',
      '.tsx',
      '.json',
      '.web.jsx',
      '.jsx',
    ]
  },
  experiments: {
    syncWebAssembly: true,
    asyncWebAssembly: true
  }
}
