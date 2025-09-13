# frozen_string_literal: true

# ENV['RUST_LOG'] ||= 'debug'

$LOAD_PATH.unshift File.expand_path("../lib", __dir__)
require "penguin"

require "minitest/autorun"
