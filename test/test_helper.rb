# frozen_string_literal: true

# Set Rust logging level for debugging
ENV['RUST_LOG'] ||= 'debug'

$LOAD_PATH.unshift File.expand_path("../lib", __dir__)
require "bson_object_id"

require "minitest/autorun"
