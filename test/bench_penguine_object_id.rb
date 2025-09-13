# frozen_string_literal: true
require "minitest/benchmark"
require "bson"
require "benchmark/ips"

require_relative "test_helper"

class BenchPenguinObjectId < Minitest::Benchmark
  def bench_linear_performance
    # The time to generate an ID should be constant so increasing the number of
    # generated IDs should increase runtime linearly.
    assert_performance_linear do |n|
      n.times { Penguin::ObjectId.new }
    end
  end
end