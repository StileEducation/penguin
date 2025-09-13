# frozen_string_literal: true
require "minitest/benchmark"
require "bson"
require "benchmark/ips"

require_relative "test_helper"

class BenchPenguinObjectId < Minitest::Benchmark
  def bench_compare_with_bson
    report = Benchmark.ips do |x|
      x.report("Penguin::ObjectId") { Penguin::ObjectId.new }
      x.report("BSON::ObjectId") { BSON::ObjectId.new }
      x.compare!
    end

    penguin = report.entries.find { |e| e.label == "Penguin::ObjectId" }
    bson = report.entries.find { |e| e.label == "BSON::ObjectId" }
    
    unless penguin.stats.overlaps?(bson.stats) || penguin.stats.central_tendency > bson.stats.central_tendency
      flunk "Penguin::ObjectId is slower than BSON::ObjectId"
    end
  end

  def bench_linear_performance
    # The time to generate an ID should be constant so increasing the number of
    # generated IDs should increase runtime linearly.
    assert_performance_linear do |n|
      n.times { Penguin::ObjectId.new }
    end
  end
end