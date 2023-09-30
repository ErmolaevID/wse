class Wse < Formula
  desc "Simple web-server. Exec wse in directory and all files are serve on localhost"
  homepage "https://github.com/ErmolaevID/wse"
  url "https://github.com/ErmolaevID/wse/releases/download/0.1.0/wse-mac.tar.gz"
  sha256 "4afe149b4c6f288b3c794b3e4f08602b9b815120f0eac19727239a15fe160d7a"
  version "0.1.0"

  def install
    bin.install "wse"
  end
end