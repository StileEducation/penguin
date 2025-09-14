# frozen_string_literal: true
#
# Tests comparing the output of Penguin::ObjectId with BSON::ObjectId.

require 'bson'
require "benchmark/ips"
require "aggregate_assertions"

require_relative 'test_helper'

class TestBsonObjectId < Minitest::Test
  def test_compare
    bson_id = BSON::ObjectId.new
    penguin_id = Penguin::ObjectId.from_time(bson_id.to_time)
    assert_equal bson_id.generation_time, penguin_id.timestamp
  end

  def test_from_time
    t = Time.now - 1000
    # BSON::ObjectId.from_time doesn't respect the input time if `unique` is
    # true so only test with `unique: false`.
    bson_id = BSON::ObjectId.from_time(t, unique: false)
    penguin_id = Penguin::ObjectId.from_time(t, unique: false)
    assert_equal bson_id.generation_time, penguin_id.timestamp
    assert_equal bson_id._counter_part.to_i(16), penguin_id.counter
  end

  def test_from_string
    string = '64c13ab08edf48a008793cac'
    bson_id = BSON::ObjectId.from_string(string)
    penguin_id = Penguin::ObjectId.from_string(string)
    assert_equal bson_id.generation_time, penguin_id.timestamp
    assert_equal bson_id._counter_part.to_i(16), penguin_id.counter
    assert_equal bson_id._process_part.to_i(16), penguin_id.machine_id
  end

  def test_compare_performance
    report = Benchmark.ips do |x|
      x.report("Penguin::ObjectId") { Penguin::ObjectId.new }
      x.report("BSON::ObjectId") { BSON::ObjectId.new }
      x.compare!
    end

    penguin = report.entries.find { |e| e.label == "Penguin::ObjectId" }
    bson = report.entries.find { |e| e.label == "BSON::ObjectId" }

    report = Benchmark.ips do |x|
      x.report("Penguin::ObjectId.to_s") { Penguin::ObjectId.new.to_s }
      x.report("BSON::ObjectId.to_s") { BSON::ObjectId.new.to_s }
      x.compare!
    end

    penguin_to_s = report.entries.find { |e| e.label == "Penguin::ObjectId.to_s" }
    bson_to_s = report.entries.find { |e| e.label == "BSON::ObjectId.to_s" }

    report = Benchmark.ips do |x|
      # Construct the ID using BSON::ObjectId for both so that the blocks, as
      # much as possible, are only testing the different in performance of the
      # `from_string` methods.
      x.report("Penguin::ObjectId.from_string") { Penguin::ObjectId.from_string(BSON::ObjectId.new.to_s) }
      x.report("BSON::ObjectId.from_string") { BSON::ObjectId.from_string(BSON::ObjectId.new.to_s) }
      x.compare!
    end

    penguin_from_string = report.entries.find { |e| e.label == "Penguin::ObjectId.from_string" }
    bson_from_string = report.entries.find { |e| e.label == "BSON::ObjectId.from_string" }

    aggregate_assertions do
      assert penguin.stats.overlaps?(bson.stats) ||
        penguin.stats.central_tendency > bson.stats.central_tendency,
        "Penguin::ObjectId is slower than BSON::ObjectId"

      assert penguin_to_s.stats.overlaps?(bson_to_s.stats) ||
        penguin_to_s.stats.central_tendency > bson_to_s.stats.central_tendency,
        "Penguin::ObjectId string generation is slower than BSON::ObjectId"

      assert penguin_from_string.stats.overlaps?(bson_from_string.stats) ||
        penguin_from_string.stats.central_tendency > bson_from_string.stats.central_tendency,
        "Penguin::ObjectId string parsing is slower than BSON::ObjectId"
    end
  end
end